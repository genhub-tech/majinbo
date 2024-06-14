#!/bin/bash

# Verificar el estado de los builds
echo "Verificando el estado de los builds..."
gh run list --limit 10

# Verificar el estado de los releases
echo "Verificando el estado de los releases..."
gh release list