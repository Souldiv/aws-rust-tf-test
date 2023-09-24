terraform {
  cloud {
    organization = "souldiv"
    workspaces {
      name = "rocket-rust-server-ec2"
    }
  }
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "rust-server" {
  ami           = "ami-03f65b8614a860c29"
  instance_type = "t2.micro"
  user_data     = file("setup_files/init.sh")
}