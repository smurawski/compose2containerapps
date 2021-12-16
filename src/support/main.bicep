targetScope = 'subscription'

// Resource Group Name
@description('Resource Group Name')
@minLength(4)
@maxLength(64)
param rgName string

// Location
@description('Location of Azure Resources')
@allowed([
  'eastus'
  'westus'
  'centralus'
])
param location string

// Container App Env Name
@description('Container App Env Name')
@minLength(4)
@maxLength(64)
param name string




resource rg 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: rgName
  location: location
}

module logAnalytics 'modules/createLogAnalytics.bicep' = {
  scope: resourceGroup(rg.name)
  name: 'logAnalyticsWorkspace'
  params: {
    name: name
  }
}

module containerAppEnv 'modules/createContainerAppEnv.bicep' = {
  scope: resourceGroup(rg.name)
  name: name
  dependsOn:[
    logAnalytics
  ]
  params: {
    name: name
    workspaceClientId: logAnalytics.outputs.clientId
    workspaceClientSecret: logAnalytics.outputs.clientSecret
  }
}

output containerappEnvId string = containerAppEnv.outputs.id
