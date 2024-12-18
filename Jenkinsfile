pipeline {
    agent any

    environment {
        DOCKER_IMAGE = 'cubejs/cube:latest'           // Official Docker image
        CUSTOM_IMAGE = 'thulasiramteja/cubejs:latest' // Replace with your DockerHub username
        GITHUB_REPO = 'https://github.com/Thulasiramtejavegi/cube.git'
        SONARQUBE_URL = 'http://192.168.0.113:9000'
        SONARQUBE_CREDENTIALS = 'sonarqube-token'     // Jenkins credentials ID for SonarQube
        DOCKERHUB_CREDENTIALS = 'dockerhub-credentials' // Jenkins credentials ID for DockerHub
    }

    stages {
        stage('Clean Workspace') {
            steps {
                script {
                    deleteDir() // Clean workspace before each build
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

        stage('Push Docker Image to DockerHub') {
            steps {
                script {
                    withCredentials([usernamePassword(credentialsId: 'dockerhub-credentials', usernameVariable: 'DOCKERHUB_USERNAME', passwordVariable: 'DOCKERHUB_PASSWORD')]) {
                        sh """
                            docker pull ${DOCKER_IMAGE}
                            docker tag ${DOCKER_IMAGE} ${CUSTOM_IMAGE}
                            echo ${DOCKERHUB_PASSWORD} | docker login -u ${DOCKERHUB_USERNAME} --password-stdin
                            docker push ${CUSTOM_IMAGE}
                        """
                    }
                }
            }
        }

        stage('Update Kubernetes Deployment YAML') {
            steps {
                script {
                    sh """
                        sed -i 's|image: .*|image: ${CUSTOM_IMAGE}|' manifests/deployment.yaml
                        git config user.name "Thulasiramtejavegi"
                        git config user.email "thulasiramteja.vegi@grooveinnovations.ai"
                        git add manifests/deployment.yaml
                        git commit -m "Update deployment.yaml to use DockerHub image ${CUSTOM_IMAGE}" || true
                        git push https://${env.GITHUB_USERNAME}:${env.GITHUB_TOKEN}@github.com/Thulasiramtejavegi/cube.git HEAD:${env.BRANCH_NAME} || true
                    """
                }
            }
        }
    }
}

.
