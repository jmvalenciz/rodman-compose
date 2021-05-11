use serde::Deserialize;
use std::env;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::config::UpArgument;
use colored::*;

#[derive(Debug, Deserialize)]
struct Service{
    image: String,
    #[serde(default)]
    ports: Vec<String>,
    #[serde(default)]
    volumes: Vec<String>,
    #[serde(default)]
    environment:HashMap<String, String>,
    #[serde(default)]
    restart: String,
}

#[derive(Debug, Deserialize)]
pub struct ContainerCompose{
    #[serde(default)]
    version: String,
    #[serde(default)]
    services: HashMap<String, Service>,
    #[serde(skip_deserializing)]
    pod_name: String,
    #[serde(skip_deserializing)]
    network: bool,
    #[serde(skip_deserializing)]
    detach: bool
}

impl ContainerCompose{
    pub fn new(yaml_str: String)->Self{
        let mut container_compose: ContainerCompose = serde_yaml::from_str(&yaml_str[..])
            .expect("Error: Unable to create Container Compose Instance:\n");
        container_compose.network = false;
        container_compose.detach = false;
        container_compose.pod_name = String::from(env::current_dir()
            .expect("Unable to get current directory")
            .file_name()
            .expect("Unable to get current directory")
            .to_str()
            .expect("Unable to get current directory"));
        container_compose
    }
    pub fn up(&mut self,up_arguments: Vec<UpArgument>){
        let composer: Vec<String>;
        let containers: Vec<Vec<String>>;

        for argument in up_arguments.iter(){
            match argument{
                UpArgument::Detach =>{
                    self.detach = true;
                }
                UpArgument::Network =>{
                    self.network = true;
                }
            }
        }

        if self.network {
            composer = self.create_network();
        }
        else{
            composer = self.create_pod();
        }

        containers = self.create_containers();

        println!("{:?}", composer);
        println!("{:?}", containers);

        ContainerCompose::spawn_command("podman", composer);
        ContainerCompose::spawn_commands("podman", containers);

    }
    pub fn down(&self){

    }

    fn create_pod(&self) -> Vec<String>{
        let mut pod: Vec<String> = vec![
            "pod".to_string(), 
            "create".to_string(), 
            "--name".to_string(), 
            self.pod_name.to_string()
        ];
        for (_, service) in self.services.iter(){
            for port in service.ports.iter(){
                pod.push("-p".to_string());
                pod.push(port.to_string());
            }
        }
        pod
    }

    fn create_network(&self) -> Vec<String>{
        vec![
            "network".to_string(),
            "create".to_string(),
            format!("{}_network",self.pod_name.to_string())
        ]
    }

    fn create_containers(&self) -> Vec<Vec<String>>{
        let mut tmp_container: Vec<String>;
        let mut containers: Vec<Vec<String>> = Vec::new();
        for (key, service) in self.services.iter(){
            tmp_container = vec!["run".to_string()];
            
            for volume in service.volumes.iter(){
                tmp_container.push("--volume".to_string());
                tmp_container.push(volume.to_string());
            }
            for (env_name,env_value) in service.environment.iter(){
                tmp_container.push("-e".to_string());
                tmp_container.push(format!("{}={}", env_name, env_value));
            }

            if service.restart != ""{
                tmp_container.push("--restart".to_string());
                tmp_container.push(service.restart.to_string());
            }
            if self.network{

                tmp_container.push("--name".to_string());
                tmp_container.push(key.to_string());

                tmp_container.push("--network".to_string());
                tmp_container.push(format!("{}_network",self.pod_name.to_string()));

                for port in service.ports.iter(){
                    tmp_container.push("-p".to_string());
                    tmp_container.push(port.to_string());
                }
            }
            else{
                tmp_container.push("--name".to_string());
                tmp_container.push(format!("{}_{}",self.pod_name,key));
                tmp_container.push("--pod".to_string());
                tmp_container.push(self.pod_name.to_string());
            }

            if self.detach {
                tmp_container.push("-d".to_string());
            }

            tmp_container.push(service.image.to_string());
            containers.push(tmp_container);
        }
        containers
    }
    
    fn spawn_command(cmd: &str, args: Vec<String>){
        let stdout = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to spawn this command")
            .stdout
            .expect("Could not capture standard output.");
        
        let reader = BufReader::new(stdout);

        reader.lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    }

    fn spawn_commands(cmd: &str, args_list: Vec<Vec<String>>){
        for args in args_list.into_iter(){
            ContainerCompose::spawn_command(cmd, args);
        }
    }
}