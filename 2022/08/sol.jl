input = readchomp( "input.txt" )

parse_int = Base.Fix1( parse, Int )
parse_ints = Base.Fix1( map, parse_int )

trees = transpose( reduce( hcat, map( parse_ints, collect.( split( input, "\n" ) ) ) ) )

function check_line( row, l_to_r )
    dir = l_to_r ? 1 : -1
    pos = l_to_r ? 1 : length( row )
    res = []
    tallest = -1
    for tree ∈ pos:dir:length( row ) - pos + 1
        push!( res, tallest < row[tree] )
        tallest = max( row[tree], tallest )
    end
    if dir == -1
        reverse!( res )
    end
    return res
end

function view_line( line, l_to_r )
    dir = l_to_r ? 1 : -1
    pos = l_to_r ? 1 : length( line )
    res = []
    for tree ∈ pos:dir:length( line ) - pos + 1
        view = 0
        for i ∈ pos:dir:tree - dir
            view = line[tree] > line[i] ? view + 1 : 1
        end
        push!( res, view )
    end
    if dir == -1
        reverse!( res )
    end
    return res
end

function check_forest( f, line_f)
    rows = []
    for row ∈ eachrow( trees )
        push!( rows, map( f,  line_f( row, true ), line_f( row, false) ) )
    end
    rows = reduce( hcat, rows )
    res = []
    for ( i, col ) ∈ enumerate( eachcol( trees ) )
        push!( res, map( f, selectdim(rows, 1, i),
                            map( f, line_f( col, true ), line_f( col, false ) ) ) )
    end
    return res
end

println( sum( sum.( check_forest( |, check_line) ) ) )
println( maximum( maximum.( check_forest( *, view_line ) ) ) )
