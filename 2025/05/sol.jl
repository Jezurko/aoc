input = split(readchomp("input.txt"), "\n\n")
ranges = split(input[1], "\n") |> x -> split.(x, "-") |> x -> map(r -> parse.(Int, r), x)
ingredients = split(input[2], "\n") |> x -> parse.(Int, x)

function check_ingredient(x)
    for r in ranges
        if x in range(r[1], r[2])
            return true
        end
    end
    return false
end

function part1()
    count(check_ingredient, ingredients)
end

function part2()
    sorted = sort(ranges)
    r_start = sorted[1][1]
    r_end = sorted[1][2]
    total = 0
    for r in sorted
        if r[1] > r_end
            total += r_end - r_start + 1
            r_start = r[1]
            r_end = r[2]
        else
            r_end = max(r_end, r[2])
        end
    end
    total += r_end - r_start + 1
    return total
end

println(part1())
println(part2())
