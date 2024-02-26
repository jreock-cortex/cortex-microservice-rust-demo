# Start minikube
minikube start

# Set docker env
eval $(minikube docker-env)             # Unix shells

# Build image
cd ..
docker build -t cortex-microservice-rust-demo .

# Run in Minikube
kubectl apply -f cortex-microservice-rust-demo.yaml 

# Check that it's running
kubectl get pods