pipeline {
    agent {
        docker { image 'rust:buster' }
    }
    stages {
        stage('Build project') {
            steps {
                sh 'cargo build'
            }
        }
    }
}