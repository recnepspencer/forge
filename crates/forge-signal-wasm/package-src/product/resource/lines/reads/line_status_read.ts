function readLineStatus(materialization) {
  return materialization.binding.statusSignal();
}

export { readLineStatus };
