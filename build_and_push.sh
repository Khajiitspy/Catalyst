@echo off

REM ==== API ====
cd transcend\transcend
docker build -t secondwind-api .
docker tag secondwind-api:latest magicabyss/secondwind-ap:latest
docker push magicabyss/secondwind-ap:latest

echo DONE
pause
