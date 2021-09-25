# frozen_string_literal: true

task :compile do
  sh 'cargo', 'build'
  sh 'avr-objcopy', '-O', 'ihex', 'target/avr-attiny85/debug/attiny85-rs.elf', 'target/avr-attiny85/debug/attiny85-rs.hex'
end

task :flash => :compile do
  sh './micronucleus/micronucleus.exe', '--run', '--timeout', '30', 'target/avr-attiny85/debug/attiny85-rs.hex'
end