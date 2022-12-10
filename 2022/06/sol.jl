input = readchomp( "input.txt" )

function find_marker( str, len )
    for i in 1 : length( str )
        pos = i + len - 1
        if length( Set( str[i : pos ] ) ) == len
            return pos
        end
    end
end

println( find_marker( input, 4 ) )
println( find_marker( input, 14 ) )
