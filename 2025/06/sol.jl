raw_input = split(readchomp("input.txt"), "\n")
number_input = raw_input[1:end-1]
op_input = raw_input[end]
numbers = number_input .|> (x -> split(x, " ", keepempty=false) |> x -> parse.(Int, x)) |> stack |> eachrow
numbers2 =  number_input |> stack |> eachrow .|> join .|> strip |> x->join(x, ',') |> x -> split(x, ",,") .|> x -> split(x, ',') .|> x -> parse(Int, x)

function str_to_op(str)
    if str == "*"
        return (*)
    elseif str == "+"
        return (+)
    end
end

operations =  op_input |> x->split(x, " ", keepempty=false) |> x -> str_to_op.(x)

function solve(ops, nums)
    zip(ops, nums) .|> (x -> splat(foldl)(x)) |> sum
end
function part1()
    solve(operations, numbers)
end

function part2()
    solve(operations, numbers2)
end

println(part1())
println(part2())
