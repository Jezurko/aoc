input = readchomp( "input.txt" )
commands = split( input, "\n" )

function cycles()
    x = 1
    reg_vals = [ x ]
    for command ∈ commands
        push!( reg_vals, x )
        if startswith( command, "addx" )
            x += parse( Int, split( command, " " )[end] )
            push!( reg_vals, x )
        end
    end
    return reg_vals
end

sum_important( reg_vals ) = sum( [reg_vals[i] * i for i ∈ 20:40:length( reg_vals )] )

function draw_display( reg_vals )
    for i ∈ 0:40:length( reg_vals ) - 2
        for j ∈ 1:40
            reg_vals[i + j] ∈ j-2:j ? print( '#' ) : print( '.' )
        end
        println()
    end
end

println( sum_important( cycles() ) )
draw_display( cycles() )
