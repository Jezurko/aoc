input = readchomp("input.txt")

function count_zeroes()
    pos = 50
    rotations = split(input, "\n")
    return count(==(0), map(rot -> pos = (100 + pos + (parse(Int, rot[2:end]) * (rot[1] == 'L' ? -1 : 1))) % 100, rotations))
end

function count_zeroes_passed()
    pos = 50
    rotations = split(input, "\n")
    zeroes = 0
    for rot in rotations
        old_pos = pos
        pos = pos + (parse(Int, rot[2:end]) * (rot[1] == 'L' ? -1 : 1))
        if (pos >= 100)
            zeroes += div(pos, 100)
        end
        if (pos <= 0)
            steps = -div(pos, 100)
            zeroes += steps
            if (old_pos > 0)
               zeroes += 1
            end
            pos += (steps + 1) * 100
        end
        pos = pos % 100
    end
    return zeroes
end

println(count_zeroes())
println(count_zeroes_passed())

