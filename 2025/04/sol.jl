input = readchomp("input.txt")
map = split(input, "\n") |> stack

function check_roll(x, y, map)
    rolls = 0
    for i in range(max(1, x - 1), min(x + 1, size(map)[1]))
        for j in range(max(1, y - 1), min(y + 1, size(map)[2]))
            if (map[i, j] == '@')
                rolls += 1
            end
        end
    end
    return rolls < 5
end

function part1()
    total = 0
    for x in range(1, size(map)[1])
        for y in range(1, size(map)[2])
            if map[x, y] == '@' && check_roll(x, y, map)
                total += 1
            end
        end
    end
    return total
end

function part2()
    total = 0
    subtotal = 1
    while (subtotal > 0)
        subtotal = 0
        for x in range(1, size(map)[1])
            for y in range(1, size(map)[2])
                if map[x, y] == '@' && check_roll(x, y, map)
                    subtotal += 1
                    map[x, y] = '.'
                end
            end
        end
        total += subtotal
    end
    return total
end

println(part1())
println(part2())
