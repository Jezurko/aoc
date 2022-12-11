monkeys = split( readchomp( "input.txt" ), "\n\n" )

mutable struct monkey
    items::Array{Int}
    op::Function
    test::Int
    true_tgt::Int
    false_tgt::Int
    inspections::Int
end

function parse_monkey( description )
    items = []; op = x->x; test = 1; true_tgt = 1; false_tgt = 1
    for line ∈ eachline( IOBuffer( description ) )
        if startswith( line, "  Starting items:" )
            items = parse.(Int, split( split( line, ":" )[end], "," ) )
        end
        if startswith( line, "  Operation:" )
            equation = split( line, "=" )[end]
            rhs = tryparse( Int, split( equation, ['*', '+'] )[end] )
            if contains( equation , '+' )
                f = +
            else
                f = *
            end
            op = rhs === nothing ? x -> f( x, x ) : x -> f( x, rhs )
        end
        if startswith( line, "  Test:" )
            test = parse( Int, split( line, "by" )[end] )
        end
        if startswith( line, "    If true:" )
            true_tgt = parse( Int, split( line, "monkey" )[end] ) + 1
        end
        if startswith( line, "    If false:" )
            false_tgt = parse( Int, split( line, "monkey" )[end] ) + 1
        end
    end
    return monkey( items, op, test, true_tgt, false_tgt, 0 )
end

function create_monkeys( monkeys_str )
    monkeys = []
    for monkey_str ∈ monkeys_str
        push!( monkeys, parse_monkey( monkey_str ) )
    end
    return monkeys
end

function play( rounds, monkeys, div )
    multi = lcm(map( x->x.test, monkeys ))
    for i ∈ 1:rounds
        for monkey ∈ monkeys
            for item ∈ monkey.items
                ins_item = ( monkey.op( item ) % multi ) ÷ div
                tgt = ins_item % monkey.test == 0 ? monkey.true_tgt : monkey.false_tgt
                push!( monkeys[tgt].items, ins_item )
                monkey.inspections += 1
            end
            monkey.items = []
        end
    end
    return monkeys
end

get_insp( monkeys ) = map( x->x.inspections, monkeys )

println( prod( partialsort( play( 20, create_monkeys( monkeys ), 3 ) |> get_insp, 1:2; rev = true ) ) )
println( prod( partialsort( play( 10000, create_monkeys( monkeys ), 1 ) |> get_insp, 1:2; rev = true ) ) )
