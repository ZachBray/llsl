#!/bin/sh
cd "$(dirname "$0")/../.."
(cd runtime/javascript && npm run build && npm link)
