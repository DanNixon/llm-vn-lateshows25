#!/usr/bin/env bash

for file in ./*.Modelfile; do
  model="$(basename "${file}" .Modelfile)"
  echo "Model \"${model}\" from ${file}"

  ollama rm "${model}"
  ollama create "${model}" -f "${file}"
done
