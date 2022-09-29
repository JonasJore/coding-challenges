function filter_list(l) {
  return l.filter((value) => typeof value !== 'string');
}

console.log(
  filter_list([1,"02", 100, "jonas"])
)