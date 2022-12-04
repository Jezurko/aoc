input = readchomp( "input.txt" )

pairs() = split.( split( input, "\n" ), "," )

makeSets( pair ) = map( x->BitSet( parse(Int, x[begin]) : parse(Int, x[end]) ), split.( pair, "-" ) )

function check_pair( pair )
    Sets = makeSets( pair )
    Sets[begin] ⊆ Sets[end] || Sets[end] ⊆ Sets[begin]
end

function check_overlap( pair )
    Sets = makeSets( pair )
    !isempty( Sets[begin] ∩ Sets[end] )
end

count_subsets() = sum( map( check_pair, pairs() ) )
count_overlaps() = sum( map( check_overlap, pairs() ) )
println( count_subsets() )
println( count_overlaps() )




