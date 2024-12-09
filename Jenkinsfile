pipeline {
    agent any

    environment {
        // Docker image details
        DOCKER_IMAGE = 'thulasiramteja/cubejs'  // Replace with your Docker Hub username/repository
        DOCKER_TAG = "${env.BRANCH_NAME}-${env.BUILD_NUMBER}"  // Use branch and build number for tag
        GITHUB_REPO = 'https://github.com/Thulasiramtejavegi/cube.git'  // Replace with your GitHub repo URL
        SONARQUBE_URL = 'http://192.168.0.109:9000'  // Replace with your actual SonarQube URL
        SONARQUBE_CREDENTIALS = 'sonarqube-token'  // SonarQube credentials ID
    }

    stages {
        stage('Clone Repository') {
            steps {
                script {
                    // Clone the GitHub repository with the provided credentials
                    git credentialsId: 'github-credentials', url: "${GITHUB_REPO}"
                }
            }
        }

        stage('Install Dependencies') {
            steps {
                script {
                    // Install dependencies for Cube.js application   // Use npm install with --legacy-peer-deps to bypass the TypeScript version conflict
                    sh 'npm install --legacy-peer-deps'
                }
            }
        }

        stage('Build Cube.js Application') {
            steps {
                script {
                    // Build the Cube.js application
                    sh 'npm run build'  // Or use the appropriate build command for Cube.js
                }
            }
        }

        stage('SonarQube Analysis') {
            steps {
                script {
                    // Perform SonarQube analysis with the provided credentials
                    withSonarQubeEnv('SonarQube') {
                        sh '/opt/sonar-scanner/bin/sonar-scanner -Dsonar.projectKey=cubejs -Dsonar.sources=src'
                    }
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    // Build the Docker image
                    sh """
                        docker build -t $DOCKER_IMAGE:$DOCKER_TAG .
                    """
                }
            }
        }

        stage('Push Docker Image to Docker Hub') {
            steps {
                script {
                    // Log in to Docker Hub and push the image
                    docker.withRegistry('https://index.docker.io/v1/', 'dockerhub-credentials') {
                        sh """
                            docker push $DOCKER_IMAGE:$DOCKER_TAG
                        """
                    }
                }
            }
        }

        stage('Update Kubernetes Deployment YAML') {
            steps {
                script {
                    // Update the Docker image in the Kubernetes deployment YAML
                    def newImage = "$DOCKER_IMAGE:$DOCKER_TAG"
                    sh """
                        sed -i 's|image: .*|image: $newImage|' manifests/deployment.yaml
                    """
                    // Commit the updated deployment.yaml back to the GitHub repository
                    sh '''
                        git config user.name "Thulasiramtejavegi"
                        git config user.email "thulasiramteja.vegi@grooveinnovations.ai"
                        git add manifests/deployment.yaml
                        git commit -m "Update deployment.yaml with new Docker image $newImage"
                        git push origin HEAD:master  # Adjust if your default branch is different
                    '''
                }
            }
        }
    }
}
