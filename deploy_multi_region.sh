#!/bin/bash

################################################################################
# ANTIMONY 2.0 - Multi-Region Deployment Script
# Phase 8: Mainnet Preparation
#
# Purpose: Deploy nodes across multiple cloud regions for geographic distribution
# Supports: AWS, GCP, Azure regions
#
# Usage: ./deploy_multi_region.sh
################################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Deployment regions
declare -A REGIONS=(
    ["aws-us-east-1"]="US East (N. Virginia)"
    ["aws-us-west-2"]="US West (Oregon)"
    ["aws-eu-west-1"]="EU (Ireland)"
    ["aws-ap-southeast-1"]="Asia Pacific (Singapore)"
    ["aws-sa-east-1"]="South America (São Paulo)"
    ["gcp-us-central1"]="GCP US Central (Iowa)"
    ["gcp-europe-west1"]="GCP Europe West (Belgium)"
    ["azure-eastus"]="Azure East US"
    ["azure-westeurope"]="Azure West Europe"
    ["azure-southeastasia"]="Azure Southeast Asia"
)

# Node specifications
declare -A NODE_SPECS=(
    ["bootstrap"]="t3.medium"    # 2 vCPU, 4GB RAM
    ["full-node"]="t3.large"     # 2 vCPU, 8GB RAM
    ["validator"]="t3.medium"    # 2 vCPU, 4GB RAM
)

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}ANTIMONY 2.0 - Multi-Region Deployment${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

################################################################################
# Deployment Plan
################################################################################
echo -e "${YELLOW}Deployment Plan:${NC}"
echo "  Bootstrap Nodes: 3"
echo "  Full Nodes: 5"
echo "  Validator Nodes: 2"
echo "  Total Regions: ${#REGIONS[@]}"
echo ""

################################################################################
# Region 1: AWS US East - Bootstrap Node
################################################################################
deploy_region_1() {
    echo -e "${BLUE}[Region 1/10]${NC} Deploying to ${REGIONS[aws-us-east-1]}..."
    
    cat > /tmp/region1-config.yaml <<EOF
provider: aws
region: us-east-1
node_type: bootstrap
instance_type: ${NODE_SPECS[bootstrap]}
port: 19000
bootstrap_peers: []
EOF
    
    echo -e "${GREEN}✓ Region 1 configured${NC}"
    echo "  Provider: AWS"
    echo "  Region: us-east-1"
    echo "  Type: Bootstrap Node"
    echo "  Port: 19000"
    echo ""
}

################################################################################
# Region 2: AWS US West - Full Node
################################################################################
deploy_region_2() {
    echo -e "${BLUE}[Region 2/10]${NC} Deploying to ${REGIONS[aws-us-west-2]}..."
    
    cat > /tmp/region2-config.yaml <<EOF
provider: aws
region: us-west-2
node_type: full-node
instance_type: ${NODE_SPECS[full-node]}
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 2 configured${NC}"
    echo "  Provider: AWS"
    echo "  Region: us-west-2"
    echo "  Type: Full Node"
    echo "  Bootstrap: Region 1"
    echo ""
}

################################################################################
# Region 3: AWS EU West - Full Node
################################################################################
deploy_region_3() {
    echo -e "${BLUE}[Region 3/10]${NC} Deploying to ${REGIONS[aws-eu-west-1]}..."
    
    cat > /tmp/region3-config.yaml <<EOF
provider: aws
region: eu-west-1
node_type: full-node
instance_type: ${NODE_SPECS[full-node]}
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION2_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 3 configured${NC}"
    echo "  Provider: AWS"
    echo "  Region: eu-west-1"
    echo "  Type: Full Node"
    echo "  Bootstrap: Regions 1, 2"
    echo ""
}

################################################################################
# Region 4: AWS Asia Pacific - Bootstrap Node
################################################################################
deploy_region_4() {
    echo -e "${BLUE}[Region 4/10]${NC} Deploying to ${REGIONS[aws-ap-southeast-1]}..."
    
    cat > /tmp/region4-config.yaml <<EOF
provider: aws
region: ap-southeast-1
node_type: bootstrap
instance_type: ${NODE_SPECS[bootstrap]}
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 4 configured${NC}"
    echo "  Provider: AWS"
    echo "  Region: ap-southeast-1"
    echo "  Type: Bootstrap Node"
    echo "  Bootstrap: Region 1"
    echo ""
}

################################################################################
# Region 5: AWS South America - Validator
################################################################################
deploy_region_5() {
    echo -e "${BLUE}[Region 5/10]${NC} Deploying to ${REGIONS[aws-sa-east-1]}..."
    
    cat > /tmp/region5-config.yaml <<EOF
provider: aws
region: sa-east-1
node_type: validator
instance_type: ${NODE_SPECS[validator]}
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION4_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 5 configured${NC}"
    echo "  Provider: AWS"
    echo "  Region: sa-east-1"
    echo "  Type: Validator"
    echo "  Bootstrap: Regions 1, 4"
    echo ""
}

################################################################################
# Region 6: GCP US Central - Full Node
################################################################################
deploy_region_6() {
    echo -e "${BLUE}[Region 6/10]${NC} Deploying to ${REGIONS[gcp-us-central1]}..."
    
    cat > /tmp/region6-config.yaml <<EOF
provider: gcp
region: us-central1
node_type: full-node
instance_type: n1-standard-2
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION2_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 6 configured${NC}"
    echo "  Provider: GCP"
    echo "  Region: us-central1"
    echo "  Type: Full Node"
    echo "  Bootstrap: Regions 1, 2"
    echo ""
}

################################################################################
# Region 7: GCP Europe West - Full Node
################################################################################
deploy_region_7() {
    echo -e "${BLUE}[Region 7/10]${NC} Deploying to ${REGIONS[gcp-europe-west1]}..."
    
    cat > /tmp/region7-config.yaml <<EOF
provider: gcp
region: europe-west1
node_type: full-node
instance_type: n1-standard-2
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION3_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 7 configured${NC}"
    echo "  Provider: GCP"
    echo "  Region: europe-west1"
    echo "  Type: Full Node"
    echo "  Bootstrap: Regions 1, 3"
    echo ""
}

################################################################################
# Region 8: Azure East US - Bootstrap Node
################################################################################
deploy_region_8() {
    echo -e "${BLUE}[Region 8/10]${NC} Deploying to ${REGIONS[azure-eastus]}..."
    
    cat > /tmp/region8-config.yaml <<EOF
provider: azure
region: eastus
node_type: bootstrap
instance_type: Standard_B2s
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 8 configured${NC}"
    echo "  Provider: Azure"
    echo "  Region: eastus"
    echo "  Type: Bootstrap Node"
    echo "  Bootstrap: Region 1"
    echo ""
}

################################################################################
# Region 9: Azure West Europe - Validator
################################################################################
deploy_region_9() {
    echo -e "${BLUE}[Region 9/10]${NC} Deploying to ${REGIONS[azure-westeurope]}..."
    
    cat > /tmp/region9-config.yaml <<EOF
provider: azure
region: westeurope
node_type: validator
instance_type: Standard_B2s
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION3_IP:19000
  - REGION7_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 9 configured${NC}"
    echo "  Provider: Azure"
    echo "  Region: westeurope"
    echo "  Type: Validator"
    echo "  Bootstrap: Regions 1, 3, 7"
    echo ""
}

################################################################################
# Region 10: Azure Southeast Asia - Full Node
################################################################################
deploy_region_10() {
    echo -e "${BLUE}[Region 10/10]${NC} Deploying to ${REGIONS[azure-southeastasia]}..."
    
    cat > /tmp/region10-config.yaml <<EOF
provider: azure
region: southeastasia
node_type: full-node
instance_type: Standard_B2ms
port: 19000
bootstrap_peers:
  - REGION1_IP:19000
  - REGION4_IP:19000
EOF
    
    echo -e "${GREEN}✓ Region 10 configured${NC}"
    echo "  Provider: Azure"
    echo "  Region: southeastasia"
    echo "  Type: Full Node"
    echo "  Bootstrap: Regions 1, 4"
    echo ""
}

################################################################################
# Generate Terraform Configuration
################################################################################
generate_terraform() {
    echo -e "${YELLOW}Generating Terraform configuration...${NC}"
    
    mkdir -p /tmp/atmn-terraform
    
    cat > /tmp/atmn-terraform/main.tf <<'EOF'
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

# AWS Provider
provider "aws" {
  region = var.aws_region
}

# GCP Provider
provider "google" {
  project = var.gcp_project
  region  = var.gcp_region
}

# Azure Provider
provider "azurerm" {
  features {}
}

# Variables
variable "aws_region" {
  default = "us-east-1"
}

variable "gcp_project" {
  default = "atmn-mainnet"
}

variable "gcp_region" {
  default = "us-central1"
}

# AWS Instance
resource "aws_instance" "atmn_node" {
  ami           = "ami-0c55b159cbfafe1f0" # Ubuntu 22.04 LTS
  instance_type = "t3.medium"
  
  user_data = file("deploy_cloud_node.sh")
  
  tags = {
    Name = "ATMN-Node"
    Type = "Bootstrap"
  }
}

# GCP Instance
resource "google_compute_instance" "atmn_node" {
  name         = "atmn-node-gcp"
  machine_type = "n1-standard-2"
  zone         = "${var.gcp_region}-a"
  
  boot_disk {
    initialize_params {
      image = "ubuntu-os-cloud/ubuntu-2204-lts"
    }
  }
  
  network_interface {
    network = "default"
    access_config {}
  }
  
  metadata_startup_script = file("deploy_cloud_node.sh")
}

# Azure VM
resource "azurerm_linux_virtual_machine" "atmn_node" {
  name                = "atmn-node-azure"
  resource_group_name = azurerm_resource_group.atmn.name
  location            = "East US"
  size                = "Standard_B2s"
  
  admin_username = "ubuntu"
  
  network_interface_ids = [
    azurerm_network_interface.atmn.id,
  ]
  
  os_disk {
    caching              = "ReadWrite"
    storage_account_type = "Standard_LRS"
  }
  
  source_image_reference {
    publisher = "Canonical"
    offer     = "0001-com-ubuntu-server-jammy"
    sku       = "22_04-lts"
    version   = "latest"
  }
  
  custom_data = filebase64("deploy_cloud_node.sh")
}
EOF
    
    echo -e "${GREEN}✓ Terraform configuration generated${NC}"
    echo "  Location: /tmp/atmn-terraform/main.tf"
    echo ""
}

################################################################################
# Generate Ansible Playbook
################################################################################
generate_ansible() {
    echo -e "${YELLOW}Generating Ansible playbook...${NC}"
    
    mkdir -p /tmp/atmn-ansible
    
    cat > /tmp/atmn-ansible/deploy.yml <<'EOF'
---
- name: Deploy ATMN Nodes
  hosts: all
  become: yes
  vars:
    atmn_version: "2.0.0"
    atmn_repo: "https://github.com/msrusu87-web/antimony-2.0.git"
    atmn_home: "/opt/atmn"
    
  tasks:
    - name: Update apt cache
      apt:
        update_cache: yes
        
    - name: Install dependencies
      apt:
        name:
          - build-essential
          - pkg-config
          - libssl-dev
          - librocksdb-dev
          - git
          - curl
        state: present
        
    - name: Install Rust
      shell: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
      args:
        creates: ~/.cargo/bin/rustc
        
    - name: Clone ATMN repository
      git:
        repo: "{{ atmn_repo }}"
        dest: "{{ atmn_home }}"
        version: main
        
    - name: Build ATMN binaries
      shell: |
        source $HOME/.cargo/env
        cd {{ atmn_home }}/atmn-core && cargo build --release
        cd {{ atmn_home }}/atmn-node && cargo build --release
      args:
        creates: "{{ atmn_home }}/atmn-core/target/release/atmn-miner"
        
    - name: Configure systemd service
      template:
        src: atmn-node.service.j2
        dest: /etc/systemd/system/atmn-node.service
        
    - name: Start ATMN node
      systemd:
        name: atmn-node
        state: started
        enabled: yes
        daemon_reload: yes
EOF
    
    cat > /tmp/atmn-ansible/inventory.ini <<'EOF'
[bootstrap]
node1 ansible_host=REGION1_IP ansible_user=ubuntu

[full_nodes]
node2 ansible_host=REGION2_IP ansible_user=ubuntu
node3 ansible_host=REGION3_IP ansible_user=ubuntu
node4 ansible_host=REGION6_IP ansible_user=ubuntu
node5 ansible_host=REGION7_IP ansible_user=ubuntu
node6 ansible_host=REGION10_IP ansible_user=ubuntu

[validators]
node7 ansible_host=REGION5_IP ansible_user=ubuntu
node8 ansible_host=REGION9_IP ansible_user=ubuntu

[bootstrap_secondary]
node9 ansible_host=REGION4_IP ansible_user=ubuntu
node10 ansible_host=REGION8_IP ansible_user=ubuntu
EOF
    
    echo -e "${GREEN}✓ Ansible playbook generated${NC}"
    echo "  Location: /tmp/atmn-ansible/deploy.yml"
    echo "  Inventory: /tmp/atmn-ansible/inventory.ini"
    echo ""
}

################################################################################
# Display Summary
################################################################################
display_summary() {
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}MULTI-REGION DEPLOYMENT PLAN${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo ""
    echo -e "${YELLOW}Network Topology:${NC}"
    echo "  Bootstrap Nodes: 3 (Regions 1, 4, 8)"
    echo "  Full Nodes: 5 (Regions 2, 3, 6, 7, 10)"
    echo "  Validator Nodes: 2 (Regions 5, 9)"
    echo "  Total Nodes: 10"
    echo ""
    echo -e "${YELLOW}Geographic Distribution:${NC}"
    echo "  North America: 3 nodes (AWS, GCP, Azure)"
    echo "  Europe: 3 nodes (AWS, GCP, Azure)"
    echo "  Asia Pacific: 2 nodes (AWS, Azure)"
    echo "  South America: 1 node (AWS)"
    echo "  Africa: 1 node (Azure planned)"
    echo ""
    echo -e "${YELLOW}Deployment Tools Generated:${NC}"
    echo "  ✓ Terraform configuration (/tmp/atmn-terraform/)"
    echo "  ✓ Ansible playbook (/tmp/atmn-ansible/)"
    echo "  ✓ Region configs (/tmp/region*-config.yaml)"
    echo ""
    echo -e "${YELLOW}Next Steps:${NC}"
    echo "  1. Update REGION*_IP placeholders with actual IPs"
    echo "  2. Deploy using: terraform apply (or) ansible-playbook deploy.yml"
    echo "  3. Monitor deployment: ./monitor_multi_region.sh"
    echo "  4. Run tests: ./test_multi_region.sh"
    echo ""
    echo -e "${GREEN}Multi-region deployment plan ready!${NC}"
    echo -e "${BLUE}============================================${NC}"
}

################################################################################
# Main Execution
################################################################################
main() {
    deploy_region_1
    deploy_region_2
    deploy_region_3
    deploy_region_4
    deploy_region_5
    deploy_region_6
    deploy_region_7
    deploy_region_8
    deploy_region_9
    deploy_region_10
    generate_terraform
    generate_ansible
    display_summary
}

main
