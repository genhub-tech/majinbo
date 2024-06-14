#!/bin/bash

# Obtener la lista de builds fallidos
FAILED_RUNS=$(gh run list --status failure --limit 5 --json databaseId,name,status,conclusion)

# Verificar si hay builds fallidos
if [ -z "$FAILED_RUNS" ]; then
  echo "No failed runs found."
  exit 1
fi

# Extraer el ID del primer build fallido
RUN_ID=$(echo $FAILED_RUNS | jq -r '.[0].databaseId')

# Verificar si se obtuvo un RUN_ID válido
if [ "$RUN_ID" == "null" ]; then
  echo "Failed to get a valid run ID."
  exit 1
fi

# Guardar los logs detallados en un archivo
gh run view $RUN_ID --log > build-logs.txt

# Verificar el contenido del archivo de logs
if [ ! -s build-logs.txt ]; then
  echo "Failed to retrieve logs."
  exit 1
fi

# Crear un archivo de prompt para Bito
echo "Summarize the following build logs:" > prompt.txt

# Usar Bito para resumir los logs
bito -p prompt.txt -f build-logs.txt -c context.txt > build-summary.txt

echo "Summary saved to build-summary.txt"
