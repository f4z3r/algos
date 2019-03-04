use v6.c;
use Test;
use lib 'lib';

use GWO;

plan 4;

subtest "sum of squares", {
    plan 4;

    sub objective(Real @params) returns Real {
        [+] @params.map(* ** 2);
    }

    my $problem = GWO::Problem.new(
        obj => &objective,
        max-iter => 200,
        w-count => 12,
        lb => Array[Real].new(-100, -100, -100),
        ub => Array[Real].new(100, 100, 100),
    );
    my @solution = $problem.solve;
    is @solution.elems, 3, "solution has correct length";
    is-approx @solution[0], 0, "solution parameter 1 is correct";
    is-approx @solution[1], 0, "solution parameter 2 is correct";
    is-approx @solution[2], 0, "solution parameter 3 is correct";

    done-testing;
}

subtest "sum of absolutes plus product of absolutes", {
    plan 2;

    sub objective(Real @params) returns Real {
        ([+] @params>>.abs) + ([*] @params>>.abs);
    }

    my $problem = GWO::Problem.new(
        obj => &objective,
        max-iter => 200,
        w-count => 8,
        lb => Array[Real].new(-100),
        ub => Array[Real].new(100),
    );
    my @solution = $problem.solve;
    is @solution.elems, 1, "solution has correct length";
    is-approx @solution[0], 0, "solution is correct";

    done-testing;
}

subtest "maximum of absolutes", {
    plan 2;

    sub objective(Real @params) returns Real {
        [max] @params>>.abs;
    }

    my $problem = GWO::Problem.new(
        obj => &objective,
        max-iter => 200,
        w-count => 20,
        lb => Array[Real].new(-100),
        ub => Array[Real].new(100),
    );
    my @solution = $problem.solve;
    is @solution.elems, 1, "solution has correct length";
    is-approx @solution[0], 0, "solution is correct";

    done-testing;
}

subtest "maximum", {
    plan 2;

    sub objective(Real @params) returns Real {
        [max] @params;
    }

    my $problem = GWO::Problem.new(
        obj => &objective,
        max-iter => 200,
        w-count => 30,
        lb => Array[Real].new(-100, -100, -100),
        ub => Array[Real].new(100, 100, 100),
    );
    my @solution = $problem.solve;
    is @solution.elems, 3, "solution has correct length";
    my $*TOLERANCE = 0.5;
    ok -100 =~= (@solution[0] | @solution[1] | @solution[2]), "solution is correct";

    done-testing;
}


done-testing;
