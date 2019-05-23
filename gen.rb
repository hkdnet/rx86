Dir['codes/*.x'].each do |filename|
  bytes = File.read(filename).each_line.flat_map do |line|
    line.split(/\s/).map { |e| eval(e) }
  end

  File.open(filename.gsub('.x', '.b'), 'wb') do |f|
    bytes.each do |b|
      f.putc(b)
    end
  end
end
