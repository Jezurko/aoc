raw_input = readchomp("input.txt")
input = raw_input |> x -> split(x, "\n") |> x -> split.(x, " ", keepempty=false) |> stack
numbers = input[:, 1:end-1] |> x -> parse.(Int, x)
numbers2 = (raw_input |> x-> split(x, "\n") .|> collect |> stack)[:, 1:end-1] |> x -> mapslices(x -> strip(join(x)), x,; dims=(2)) |> x->join(x, ',') |> x -> split(x, ",,") .|> x -> split(x, ',') .|> x -> parse(Int, x)

function str_to_op(str)
    if str == "*"
        return (*)
    elseif str == "+"
        return (+)
    end
end

operations = input[:, end:end] |> x -> str_to_op.(x)

function part1()
    zip(operations, eachrow(numbers)) .|> (x->splat(foldl)(x)) |> sum
end

function part2()
    zip(operations, numbers2) .|> (x->splat(foldl)(x)) |> sum
end

println(part1())
println(part2())
