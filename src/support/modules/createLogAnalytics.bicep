param location string = resourceGroup().location
param name string

resource workspace 'Microsoft.OperationalInsights/workspaces@2020-03-01-preview' = {
  name: name
  location: location
  properties: any({
    retentionInDays: 30
    features: {
      searchVersion: 1
    }
    sku: {
      name: 'PerGB2018'
    }
  })
}

output workspaceId string = workspace.id
output clientId string = workspace.properties.customerId
output clientSecret string = workspace.listKeys().primarySharedKey
