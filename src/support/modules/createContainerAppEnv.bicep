param name string
param location string = resourceGroup().location
param workspaceClientId string
param workspaceClientSecret string

resource env 'Microsoft.Web/kubeEnvironments@2021-02-01' = {
  name: name
  location: location
  properties: {
    type: 'managed'
    internalLoadBalancerEnabled: false
    appLogsConfiguration: {
      destination: 'log-analytics'
      logAnalyticsConfiguration: {
        customerId: workspaceClientId
        sharedKey: workspaceClientSecret
      }
    }
  }
}
output id string = env.id
