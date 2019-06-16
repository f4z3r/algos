#!/usr/bin/env perl6

use v6.c;
use strict;

unit module GWO;

my class Wolf {
    has Real @.pos = ();
    has Real $.fitness = Inf;

    submethod BUILD(Real :@lb, Real :@ub) {
        @!pos.push: ($_[0]..$_[1]).rand for @lb Z @ub;
    }

    method update-fitness(Wolf:D: Real &obj) {
        $!fitness = obj(@!pos);
    }

    method update-pos(Wolf:D: Wolf:D $alpha, Wolf:D $beta, Wolf:D $delta, Real $a) {
        for ^@!pos.elems -> $dim {
            my @positions = ($alpha, $beta, $delta).map( -> $leader {
                my $r1 = 1.rand;
                my $r2 = 1.rand;
                my $A = 2 * $a * $r1 - $a;
                my $C = 2 * $r2;
                my $D-leader = ($C * $leader.pos[$dim] - @!pos[$dim]);
                $leader.pos[$dim] - $A * $D-leader;
            });
            @!pos[$dim] = ([+] @positions) / 3;
        }
    }

    method clamp-pos(Wolf:D: Real :@lb, Real :@ub) {
        for (@lb Z @ub).kv -> $idx, @bounds {
            @!pos[$idx] = @bounds[0] if @!pos[$idx] < @bounds[0];
            @!pos[$idx] = @bounds[1] if @!pos[$idx] > @bounds[1];
        }
    }
}

my class Pack {
    has Wolf $!alpha;
    has Wolf $!beta;
    has Wolf $!delta;
    has Wolf @!w = ();

    submethod BUILD(Real :@lb, Real :@ub, Int :$w-count where { $w-count >= 3 }) {
        @!w.push: Wolf.new(:@lb, :@ub) for ^$w-count;
        $!alpha := @!w[0];
        $!beta := @!w[1];
        $!delta := @!w[2];

    }

    method update-leaders(Pack:D: &obj) {
        for @!w -> $wolf {
            $wolf.update-fitness(&obj);
            given $wolf.fitness {
                $!alpha := $wolf when $_ < $!alpha.fitness;
                $!beta := $wolf when $!alpha.fitness < $_ < $!beta.fitness;
                $!delta := $wolf when $!beta.fitness < $_ < $!delta.fitness;
            }
        }
    }

    method update-positions(Pack:D: Real $a) {
        .update-pos($!alpha, $!beta, $!delta, $a) for @!w;
    }

    method clamp-position(Pack:D: Real :@lb, Real :@ub) {
        .clamp-pos(:@lb, :@ub) for @!w;
    }

    method alpha-position(Pack:D:) returns Array[Real] {
        $!alpha.pos;
    }
}

class Problem {
    has Pack $!pack;
    has Real &!obj is required;
    has Int $!max-iter is required;
    has Int $!w-count is required;
    has Real @!lb is required;
    has Real @!ub is required;

    submethod BUILD(
        Real :&!obj,
        Int :$!max-iter,
        Int :$!w-count,
        :@!lb,
        :@!ub where { @!ub ~~ Array[Real] and @!lb ~~ Array[Real] }
    ) {}


    submethod TWEAK {
        $!pack = Pack.new(:@!lb, :@!ub, :$!w-count);
    }

    method solve(Problem:D:) returns Array[Real] {
        for ^$!max-iter -> $iter {
            $!pack.update-leaders(&!obj);
            my $a = 2 - $iter * (2 / $!max-iter);
            $!pack.update-positions($a);
            $!pack.clamp-position(:@!lb, :@!ub);
        }
        $!pack.alpha-position;
    }
}
