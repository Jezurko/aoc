input = readchomp( "input.txt" )

pairs = split( input, "\n\n" )

function check( arr1::Integer, arr2::Integer )
    return arr2 - arr1
end

function check( arr1::Integer, arr2::AbstractArray )
    return check( [arr1], arr2 )
end

function check( arr1::AbstractArray, arr2::Integer )
    return check( arr1, [arr2] )
end

function check( arr1::AbstractArray, arr2::AbstractArray )
    for ( left, right ) ∈ zip( arr1, arr2 )
        res = check( left, right )
        if res != 0
            return res
        end
    end
    return length( arr2 ) - length( arr1 )
end

function correct_pairs()
    sum = 0
    for ( i , pair ) ∈ enumerate( pairs )
        ( left, right ) = split( pair, "\n" )
        res = check( eval( Meta.parse( left ) ), eval( Meta.parse( right ) ) )
        if res > 0
            sum += i
        end
    end
    return sum
end

function decode()
    first = [[2]]
    second = [[6]]
    packets = eval.( Meta.parse.(split( input ) ) )
    push!( packets, [[2]], [[6]] )
    sort!( packets; lt = (a, b) -> check(a, b) > 0 )
    return findfirst( x -> x == first, packets) * findfirst( x -> x == second, packets )
end

println( "part1: ", correct_pairs() )
println( "part2: ", decode() )
