extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::env;

async fn pods(){
    println!("A pod is the smallest execution unit in Kubernetes. A pod encapsulates one or more applications");
    let s = 
    "
    apiVersion: v1
    kind: Pod
    metadata:
        name: nginx
    spec:
        containers:
        - name: nginx
          image: nginx:1.14.2
          ports:
          - containerPort: 80";
    let pod_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &pod_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/workloads/pods/")
}

async fn deployments(){
    println!("A Kubernetes Deployment tells Kubernetes how to create or modify instances of the pods that hold a containerized application.");
    let s = 
    "
    apiVersion: apps/v1
    kind: Deployment
    metadata:
        name: nginx-deployment
        labels:
            app: nginx
    spec:
        replicas: 3
        selector:
            matchLabels:
                app: nginx
        template:
            metadata:
                labels:
                    app: nginx
            spec:
                containers:
                - name: nginx
                  image: nginx:1.14.2
                  ports:
                  - containerPort: 80";

    let deployment_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &deployment_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/workloads/controllers/deployment/")
}

async fn services(){
    println!("A service is a logical collection of pods in a Kubernetes cluster. We can define a K8s service as an abstract way to load balance across the pods and expose an application deployed on a set of pods");
    let s = 
    "
    apiVersion: v1
    kind: Service
    metadata:
        name: my-service
    spec:
        selector:
            app.kubernetes.io/name: MyApp
        ports:
        - protocol: TCP
          port: 80
          targetPort: 9376
    ";

    let service_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &service_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/services-networking/service/")
}


async fn daemonsets(){
    println!("A daemonSet is used to ensure that some or all of our K8S nodes run a copy of a pod, which allows us to run a daemon on every node.");
    let s = 
    "
    apiVersion: apps/v1
    kind: DaemonSet
    metadata:
        name: fluentd-elasticsearch
        namespace: kube-system
        labels:
            k8s-app: fluentd-logging
    spec:
        tolerations:
        # these tolerations are to have the daemonset runnable on control plane nodes
        # remove them if your control plane nodes should not run pods
        - key: node-role.kubernetes.io/control-plane
        operator: Exists
        effect: NoSchedule
        - key: node-role.kubernetes.io/master
        operator: Exists
        effect: NoSchedule
        containers:
        - name: fluentd-elasticsearch
          image: quay.io/fluentd_elasticsearch/fluentd:v2.5.2
          resources:
            limits:
                memory: 200Mi
            requests:
                cpu: 100m
                memory: 200Mi
          volumeMounts:
          - name: varlog
            mountPath: /var/log
          - name: varlibdockercontainers
            mountPath: /var/lib/docker/containers
            readOnly: true
        terminationGracePeriodSeconds: 30
        volumes:
        - name: varlog
            hostPath:
            path: /var/log
        - name: varlibdockercontainers
            hostPath:
            path: /var/lib/docker/containers
    ";

    let daemonset_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &daemonset_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/")
}

async fn replicasets(){
    println!("A replicaret is a process that runs multiple instances of a pod and keeps the specified number of pods constant.");
    let s = 
    "
    apiVersion: apps/v1
    kind: ReplicaSet
    metadata:
        name: frontend
        labels:
            app: guestbook
            tier: frontend
    spec:
        replicas: 3
        selector:
            matchLabels:
                tier: frontend
        template:
            metadata:
                labels:
                    tier: frontend
            spec:
                containers:
                - name: php-redis
                  image: gcr.io/google_samples/gb-frontend:v3
    ";

    let replicaset_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &replicaset_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/")
}

async fn ingress(){
    println!("An ingress is an API object that describes the routing rules for traffic (typically HTTP or HTTPS) into an application running within a Kubernetes cluster");
    let s = 
    "
    apiVersion: networking.k8s.io/v1
    kind: Ingress
    metadata:
        name: minimal-ingress
    annotations:
        nginx.ingress.kubernetes.io/rewrite-target: /
    spec:
        ingressClassName: nginx-example
        rules:
        - http:
            paths:
            - path: /testpath
              pathType: Prefix
              backend:
                service:
                    name: test
                    port:
                        number: 80
    ";

    let ingress_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &ingress_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/services-networking/ingress/")
}

async fn volumes(){
    println!("A volume is a directory containing data, which can be accessed by containers in a Kubernetes pod");
    let s = 
    "
    apiVersion: v1
    kind: Pods
    metadata:
        name: test-ebs
    spec:
        containers:
        - image: registry.k8s.io/test-webserver
          name: test-container
          volumeMounts:
          - mountPath: /test-ebs
            name: test-volume
        volumes:
        - name: test-volume
          # This EBS volume must already exist.
          awsElasticBlockStore:
            volumeID: '<volumeid>'
            fsType: ext4
    ";

    let ingress_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &ingress_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/storage/volumes/")
}

async fn secrets(){
    println!("A secret is an object for storing sensitive pieces of data such as usernames, passwords, tokens, and keys");
    let s = 
    "
    apiVersion: v1
    data:
        username: YWRtaW4=
        password: MWYyZDFlMmU2N2Rm
    kind: Secret
    metadata:
        annotations:
            kubectl.kubernetes.io/last-applied-configuration: { ... }
        creationTimestamp: 2020-01-22T18:41:56Z
        name: mysecret
        namespace: default
        resourceVersion: '164619'
        uid: cfee02d6-c137-11e5-8d73-42010af00002
    type: Opaque
    ";

    let secret_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &secret_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/configuration/secret/")
}

async fn config_maps(){
    println!("A Kubernetes ConfigMap is an API object that allows you to store data as key-value pairs. Kubernetes pods can use ConfigMaps as configuration files, environment variables or command-line arguments.");
    let s = 
    "
    apiVersion: v1
    kind: ConfigMap
    metadata:
        name: game-demo
    data:
        # property-like keys; each key maps to a simple value
        player_initial_lives: '3'
        ui_properties_file_name: 'user-interface.properties'

        # file-like keys
        game.properties: |
            enemy.types=aliens,monsters
            player.maximum-lives=5    
        user-interface.properties: |
            color.good=purple
            color.bad=yellow
            allow.textmode=true
    ";

    let config_map_yaml = YamlLoader::load_from_str(s).unwrap();

    let yaml = &config_map_yaml[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(yaml).unwrap(); 
    }
    println!("{}", out_str);
    println!();
    println!("To learn more, please visit here: https://kubernetes.io/docs/concepts/configuration/secret/")
}



#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args[1] == "pods" {
        pods().await;
    }else if args[1] == "deployments" {
        deployments().await;
    }else if args[1] == "services" {
        services().await;
    }else if args[1] == "daemonsets" {
        daemonsets().await;
    }else if args[1] == "replicasets" {
        replicasets().await;
    }else if args[1] == "ingress" {
        ingress().await;
    }else if args[1] == "volumes" {
        volumes().await;
    }else if args[1] == "secrets" {
        secrets().await;
    }else if args[1] == "configmaps" {
        config_maps().await;
    }

}
