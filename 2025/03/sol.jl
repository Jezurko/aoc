input = readchomp("input.txt")

function get_max_joltage(line)
    maximum = 0
    for i in range(1, length(line))
        for j in range(i + 1, length(line))
            maximum = max(maximum, parse(Int, line[i] * line[j]))
        end
    end
    return maximum
end

function get_max_joltage_2(line)
    max_jolt = fill("0", 12, length(line))
    max_jolt[1,:] = string.(collect(line))
    for i in range(2, 12)
        for j in range(length(line) - (i - 1), 1, step = -1)
            for k in range(j + 1, length(line))
                candidate = line[j]*max_jolt[i - 1, k]
                if parse(Int, candidate) > parse(Int, max_jolt[i, j])
                    max_jolt[i, j] = candidate
                end
            end
        end
    end
    return maximum(parse.(Int, max_jolt[12,:]))
end

function part1()
    sum(get_max_joltage.(split(input, "\n")))
end

function part2()
    sum(get_max_joltage_2.(split(input, "\n")))
end

println(part1())
println(part2())
