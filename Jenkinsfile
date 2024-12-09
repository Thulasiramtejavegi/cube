pipeline {
    agent any

    environment {
        DOCKER_BUILDKIT = "1"  // Enable Docker BuildKit
        DOCKER_IMAGE = 'thulasiramteja/cubejs'
        DOCKER_TAG = "${env.BRANCH_NAME}-${env.BUILD_NUMBER}"
        GITHUB_REPO = 'https://github.com/Thulasiramtejavegi/cube.git'
        SONARQUBE_URL = 'http://192.168.0.109:9000'
        SONARQUBE_CREDENTIALS = 'sonarqube-token'
        DOCKERFILE_PATH = 'rust/cubestore/Dockerfile'  // Dockerfile path
        DOCKER_CONTEXT = '.'  // Root of the repository
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
                    git credentialsId: 'github-credentials', url: "${GITHUB_REPO}"
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
                    sh """
                        docker buildx build -f ${DOCKERFILE_PATH} -t ${DOCKER_IMAGE}:${DOCKER_TAG} ${DOCKER_CONTEXT}
                    """ || error("Docker build failed")
                }
            }
        }

        stage('Push Docker Image to Docker Hub') {
            steps {
                script {
                    docker.withRegistry('https://index.docker.io/v1/', 'dockerhub-credentials') {
                        sh "docker push ${DOCKER_IMAGE}:${DOCKER_TAG} || error('Failed to push Docker image')"
                    }
                }
            }
        }

        stage('Update Kubernetes Deployment YAML') {
            steps {
                script {
                    def newImage = "${DOCKER_IMAGE}:${DOCKER_TAG}"
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
