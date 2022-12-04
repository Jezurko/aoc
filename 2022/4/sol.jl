input = readchomp( "input.txt" )

pairs() = split.( split( input, "\n" ), "," )

make_sets( pair ) = map( x->BitSet( parse( Int, x[begin] ) : parse( Int, x[end] ) ),
                         split.( pair, "-" ) )

function check_pair( pair )
    sets = make_sets( pair )
    sets[begin] ⊆ sets[end] || sets[end] ⊆ sets[begin]
end

check_overlap( pair ) = !isdisjoint( make_sets( pair )... )

count_subsets() = sum( check_pair, pairs() )
count_overlaps() = sum( check_overlap, pairs() )

println( count_subsets() )
println( count_overlaps() )
