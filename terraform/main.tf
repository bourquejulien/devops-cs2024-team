resource "azurerm_resource_group" "rg" {
  location = var.resource_group_location
  name     = format("%s-%s-rg", "CS", var.team_name)
}

resource "azurerm_kubernetes_cluster" "team_cluster" {
  name                = "team-cluster"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
  dns_prefix          = "aicluster"

  default_node_pool {
    name       = "default"
    node_count = 1
    vm_size    = "Standard_B2s"
  }

  identity {
      type = "SystemAssigned"
  }
}
