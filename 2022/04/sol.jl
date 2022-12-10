using Combinatorics
input = readchomp( "input.txt" )

pairs() = split.( split( input, "\n" ), "," )

make_ranges( pair ) = map( x->parse( Int, x[begin] ) : parse( Int, x[end] ), split.( pair, "-" ) )

check_subset2(pair) = any( x->⊆( x... ), pair |> make_ranges |> permutations )

function check_subset( pair )
    sets = make_ranges( pair )
    sets[begin] ⊆ sets[end] || sets[end] ⊆ sets[begin]
end

check_overlap( pair ) = !isdisjoint( make_ranges( pair )... )

count_subsets() = sum( check_subset2, pairs() )
count_overlaps() = sum( check_overlap, pairs() )

println( count_subsets() )
println( count_overlaps() )
