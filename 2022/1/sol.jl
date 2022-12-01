fatelves = [0, 0, 0]
open( "input.txt" ) do file
    elf = 0
    for ln in eachline( file )
        if ( isempty( ln ) )
            global fatelves
            thinelf = argmin( fatelves )
            fatelves[ thinelf ] = max( elf, fatelves[ thinelf ] )
            elf = 0
        else
            elf += parse( Int, ln )
        end
    end
end
println( fatelves )
println( sum( fatelves ) )
