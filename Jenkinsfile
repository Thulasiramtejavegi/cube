pipeline {
    agent any

    environment {
        DOCKER_IMAGE = 'thulasiramteja/cubejs'
        DOCKER_TAG = "${env.BRANCH_NAME}-${env.BUILD_NUMBER}"
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
                    git credentialsId: 'github-credentials', url: "${GITHUB_REPO}"
                }
            }
        }

        stage('Install Dependencies') {
            steps {
                script {
                    sh 'npm install --legacy-peer-deps'
                }
            }
        }

        stage('Build Cube.js Application') {
            steps {
                script {
                    sh 'npm run build'
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
                    sh "docker build -t $DOCKER_IMAGE:$DOCKER_TAG ."
                }
            }
        }

        stage('Push Docker Image to Docker Hub') {
            steps {
                script {
                    docker.withRegistry('https://index.docker.io/v1/', 'dockerhub-credentials') {
                        sh "docker push $DOCKER_IMAGE:$DOCKER_TAG"
                    }
                }
            }
        }

        stage('Update Kubernetes Deployment YAML') {
            steps {
                script {
                    def newImage = "$DOCKER_IMAGE:$DOCKER_TAG"
                    sh "sed -i 's|image: .*|image: $newImage|' manifests/deployment.yaml"
                    sh '''
                        git config user.name "Thulasiramtejavegi"
                        git config user.email "thulasiramteja.vegi@grooveinnovations.ai"
                        git add manifests/deployment.yaml
                        git commit -m "Update deployment.yaml with new Docker image $newImage"
                        git push origin HEAD:${env.BRANCH_NAME}
                    '''
                }
            }
        }
    }
}
