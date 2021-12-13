param (
  [switch]$Create,
  [switch]$Delete,
  [switch]$Logs,
  $Name
)

$RESOURCE_GROUP="cnc"
$LOCATION="eastus"
$LOG_ANALYTICS_WORKSPACE="calogs"
$CONTAINERAPPS_ENVIRONMENT="fromcompose"


if ($Create) {
  az group create `
  --name $RESOURCE_GROUP `
  --location "$LOCATION"

  az monitor log-analytics workspace create `
    --resource-group $RESOURCE_GROUP `
    --workspace-name $LOG_ANALYTICS_WORKSPACE

  start-sleep -seconds 2
  $LOG_ANALYTICS_WORKSPACE_CLIENT_ID=(az monitor log-analytics workspace show --query customerId -g $RESOURCE_GROUP -n $LOG_ANALYTICS_WORKSPACE --out tsv)
  start-sleep -seconds 2
  $LOG_ANALYTICS_WORKSPACE_CLIENT_SECRET=(az monitor log-analytics workspace get-shared-keys --query primarySharedKey -g $RESOURCE_GROUP -n $LOG_ANALYTICS_WORKSPACE --out tsv)
  start-sleep -seconds 2

  az containerapp env create `
    --name $CONTAINERAPPS_ENVIRONMENT `
    --resource-group $RESOURCE_GROUP `
    --logs-workspace-id $LOG_ANALYTICS_WORKSPACE_CLIENT_ID `
    --logs-workspace-key $LOG_ANALYTICS_WORKSPACE_CLIENT_SECRET `
    --location "$LOCATION"
}

if ($logs) {
  start-sleep -seconds 2
  $LOG_ANALYTICS_WORKSPACE_CLIENT_ID=(az monitor log-analytics workspace show --query customerId -g $RESOURCE_GROUP -n $LOG_ANALYTICS_WORKSPACE --out tsv)
  start-sleep -seconds 2
  $LOG_ANALYTICS_WORKSPACE_CLIENT_SECRET=(az monitor log-analytics workspace get-shared-keys --query primarySharedKey -g $RESOURCE_GROUP -n $LOG_ANALYTICS_WORKSPACE --out tsv)
  start-sleep -seconds 2
  az monitor log-analytics query --workspace "$LOG_ANALYTICS_WORKSPACE_CLIENT_ID" --analytics-query "ContainerAppConsoleLogs_CL | where ContainerAppName_s == '$Name' | project ContainerAppName_s, Log_s, TimeGenerated | order by TimeGenerated asc" -o tsv

}

if ($Delete) {
  az group delete --name $RESOURCE_GROUP --no-wait -y
}