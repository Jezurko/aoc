fatelfs = [0, 0, 0]
open( "input.txt" ) do file
    elf = 0
    for ln in eachline( file )
        if ( isempty( ln ) )
            global fatelfs
            thinelf = argmin( fatelfs )
            fatelfs[ thinelf ] = max( elf, fatelfs[ thinelf ] )
            elf = 0
        else
            elf += parse( Int, ln )
        end
    end
end
println( fatelfs )
println( sum( fatelfs ) )
