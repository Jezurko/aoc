input = readchomp( "input.txt" )
lines = split( input, "\n" )

function get_grid()
    grid = Set()
    for line ∈ lines
        coords = split( line, " -> " )
        head, tail = Iterators.peel( coords )
        x, y = parse.( Int, split( head, "," ) )
        for coord ∈ tail
            new_x, new_y = parse.( Int, split( coord, "," ) )
            if x == new_x
                grid = grid ∪ Set( (x, i) for i ∈ y:sign( new_y - y ):new_y )
            else
                grid = grid ∪ Set( (i, y) for i ∈ x:sign( new_x - x ):new_x )
            end
            x = new_x
            y = new_y
        end
    end
    return grid
end

function throw_sand( grid )
    lowest = maximum( x->x[end], grid )
    grains = 0
    while( true )
        x = 500
        y = 0
        while( true )
            if y > lowest
                return grains
            end
            if (x, y + 1) ∉ grid
                y += 1
            elseif (x - 1, y + 1) ∉ grid
                x -= 1
                y += 1
            elseif (x + 1, y + 1) ∉ grid
                x += 1
                y += 1
            else
                push!( grid, (x, y) )
                grains += 1
                break
            end
        end
    end
end

function part_2( grid )
    lowest = maximum( x->x[end], grid )
    grains = 0
    while( true )
        x = 500
        y = 0
        while( true )
            if (y == lowest + 1)
                push!( grid, (x, y) )
                grains += 1
                break
            end
            if (x, y + 1) ∉ grid
                y += 1
            elseif (x - 1, y + 1) ∉ grid
                x -= 1
                y += 1
            elseif (x + 1, y + 1) ∉ grid
                x += 1
                y += 1
            else
                push!( grid, (x, y) )
                grains += 1
                break
            end
        end
        if (x, y) == (500, 0)
            return grains
        end
    end
end

println( throw_sand( get_grid() ) )
println( part_2( get_grid() ) )

