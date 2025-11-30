#!/bin/bash
# Type check script for lint-staged
# This ensures vue-tsc uses the correct tsconfig.json

cd "$(dirname "$0")/.." || exit 1
exec pnpm vue-tsc --noEmit --skipLibCheck

