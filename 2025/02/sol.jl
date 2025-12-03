input = readchomp("input.txt")

function get_ranges()
    map(x -> parse.(Int, x), map(x -> split(x, "-"), split(input, ",")))
end

function range_to_strings(id_range)
    res = []
    for id in range(id_range[1], id_range[2])
        push!(res, string(id))
    end
    return res
end

function check_repeat(str_id)
   return str_id == repeat(str_id[1:div(length(str_id), 2)], 2)
end

function check_multi_repeat(str_id)
   len = length(str_id)
   for i in range(1, div(len, 2))
       if len % i != 0
           continue
        end
        if str_id == repeat(str_id[1:i], div(len, i))
            return true
        end
   end
   return false
end

function sum_doubles(str_ids, filter_fn)
    sum(map(id -> parse(Int, id), filter(str_id -> filter_fn(str_id), str_ids)))
end

function part1()
    sum(sum_doubles.(map(range_to_strings, get_ranges()), check_repeat))
end

function part2()
    sum(sum_doubles.(map(range_to_strings, get_ranges()), check_multi_repeat))
end

println(part1())
println(part2())
