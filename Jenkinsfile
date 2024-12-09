pipeline {
    agent any

    environment {
        DOCKER_IMAGE = 'cubejs/cube:latest'  // Official Docker image
        GITHUB_REPO = 'https://github.com/Thulasiramtejavegi/cube.git'
        SONARQUBE_URL = 'http://192.168.0.109:9000'
        SONARQUBE_CREDENTIALS = 'sonarqube-token'
    }

    stages {
        stage('Clean Workspace') {
            steps {
                script {
                    deleteDir()  // Clean workspace before each build
                }
            }
        }

        stage('Clone Repository') {
            steps {
                script {
                    git(credentialsId: 'github-credentials', url: "${GITHUB_REPO}")
                }
            }
        }

        stage('Install Dependencies') {
            steps {
                script {
                    sh 'npm install --legacy-peer-deps || echo "Failed to install dependencies"'
                }
            }
        }

        stage('Build Cube.js Application') {
            steps {
                script {
                    sh 'npm run build || echo "Build failed"'
                }
            }
        }

        stage('SonarQube Analysis') {
            steps {
                script {
                    withSonarQubeEnv('SonarQube') {
                        sh '/opt/sonar-scanner/bin/sonar-scanner -Dsonar.projectKey=cubejs -Dsonar.sources=.'
                    }
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    // No Docker build step needed as we are using the official image
                }
            }
        }

        stage('Update Kubernetes Deployment YAML') {
            steps {
                script {
                    def newImage = "${DOCKER_IMAGE}"
                    sh """
                        sed -i 's|image: .*|image: ${newImage}|' manifests/deployment.yaml
                        git config user.name "Thulasiramtejavegi"
                        git config user.email "thulasiramteja.vegi@grooveinnovations.ai"
                        git add manifests/deployment.yaml
                        git commit -m "Update deployment.yaml with new Docker image ${newImage}" || error("Failed to commit changes")
                        git push origin HEAD:${env.BRANCH_NAME} || error("Failed to push changes")
                    """
                }
            }
        }
    }
}
