#!/bin/bash

cd ~/lily || exit
git add .
git commit -m "Auto-update dotfiles: $(date)"
git push origin main  # Replace 'main' with your branch name

echo "Repo pushed successfully!"

