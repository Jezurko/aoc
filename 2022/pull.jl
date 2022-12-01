using DotEnv
using HTTP

function pull()
    DotEnv.config( path = "../.env" )
    day =  Base.Filesystem.basename( pwd() )
    agent = "https://github.com/Jezurko/aoc/tree/main/2022 by " * ENV["mail"]
    res = HTTP.get( "http://adventofcode.com/" * ENV["year"] * "/day/" * day * "/input",
                   cookies=Dict( "session"=>ENV["session"] ),
                   headers=Dict( ["User-agent"=>agent] )
                   )
    out = open( "input.txt", "w" )
    println( out, String(res.body) )
end

pull()
