pages = ARGF.readlines
order = [8, 4, 7, 3, 6, 2, 5, 1].map { |i| i - 1 }

for char in 0..pages[0].size do
  for i in order do
    print pages[i][char]
  end
end
