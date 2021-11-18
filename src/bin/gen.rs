// use std::slice::range;
use rand::prelude::*;
use tsp_sa::point::*;

fn output_points(points: &Vec<Point>) {
    println!("{}", points.len());
    for i in points {
        for j in points {
            print!("{} ", i.distance(j));
        }
        println!();
    }
    // for i in 0..points.len() {
    //     println!("{} {}", points[i].x, points[i].y);
    // }
}

fn main() {
    // let points = read_points(true);
    let mut points = Vec::<Point>::new();
    let n = 22;
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        points.push(Point {
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        });
    }
    output_points(&points);
}
/*
31
0 0
0 1
0 2
0 3
0 4
0 5
0 6
0 7
0 8
0 9
0 10
1 10
2 10
3 10
4 10
5 10
6 10
7 10
8 10
9 10
10 10
9 9
8 8
7 7
6 6
5 5
6 4
7 3
8 2
9 1
10 0
31
0 8.06225774829855 3.605551275463989 2 9.433981132056603 2.23606797749979 2.8284271247461903 1.4142135623730951 6.324555320336759 5.0990195135927845 7.280109889280518 8.246211251235321 8.246211251235321 4.242640687119285 6 8 1.4142135623730951 7.0710678118654755 8.54400374531753 10 4.47213595499958 8.06225774829855 11.313708498984761 2.8284271247461903 10.63014581273465 8.246211251235321 4.47213595499958 2.8284271247461903 8.94427190999916 2.23606797749979 5.385164807134504
8.06225774829855 0 5.0990195135927845 8.06225774829855 6 7.0710678118654755 10.04987562112089 9 2.23606797749979 9.219544457292887 1.4142135623730951 1 3 6.4031242374328485 10.63014581273465 1 7.280109889280518 12.041594578792296 4 7 4.123105625617661 2 9 6.708203932499369 8 13.45362404707371 7.810249675906654 6.082762530298219 5 9.055385138137417 3.1622776601683795
3.605551275463989 5.0990195135927845 0 3 8.602325267042627 2 5 4.123105625617661 3 7.280109889280518 4 5 6.4031242374328485 5 8.54400374531753 5.385164807134504 3.605551275463989 9.848857801796104 7.0710678118654755 9.433981132056603 1 5.830951894845301 11.180339887498949 4.123105625617661 10.295630140987 11.180339887498949 6.082762530298219 1 7.810249675906654 4 2
2 8.06225774829855 3 0 10.63014581273465 1 2 1.4142135623730951 6 7.0710678118654755 7 8 8.94427190999916 5.830951894845301 8 8.246211251235321 3.1622776601683795 9.055385138137417 9.433981132056603 11.313708498984761 4 8.54400374531753 12.806248474865697 4.47213595499958 12.041594578792296 10.198039027185569 6.324555320336759 2 10 1 5
9.433981132056603 6 8.602325267042627 10.63014581273465 0 9.899494936611665 12.206555615733702 10.816653826391969 7.280109889280518 7 7.0710678118654755 7 3 5.385164807134504 8.06225774829855 5 8.06225774829855 9.219544457292887 2 1 8.06225774829855 4 3 6.708203932499369 2 10.44030650891055 6.082762530298219 9.219544457292887 1 11.40175425099138 7.615773105863909
2.23606797749979 7.0710678118654755 2 1 9.899494936611665 0 3 2.23606797749979 5 7 6 7 8.06225774829855 5.385164807134504 8.06225774829855 7.280109889280518 3 9.219544457292887 8.602325267042627 10.63014581273465 3 7.615773105863909 12.206555615733702 4.123105625617661 11.40175425099138 10.44030650891055 6.082762530298219 1 9.219544457292887 2 4
2.8284271247461903 10.04987562112089 5 2 12.206555615733702 3 0 1.4142135623730951 8 7.615773105863909 9 10 10.770329614269007 7.0710678118654755 8.246211251235321 10.198039027185569 4.242640687119285 9.055385138137417 11.180339887498949 12.806248474865697 6 10.44030650891055 14.142135623730951 5.656854249492381 13.45362404707371 10 7.211102550927978 4 11.661903789690601 1 7
1.4142135623730951 9 4.123105625617661 1.4142135623730951 10.816653826391969 2.23606797749979 1.4142135623730951 0 7.0710678118654755 6.324555320336759 8.06225774829855 9.055385138137417 9.486832980505138 5.656854249492381 7.0710678118654755 9.055385138137417 2.8284271247461903 8 9.848857801796104 11.40175425099138 5.0990195135927845 9.219544457292887 12.727922061357855 4.242640687119285 12.041594578792296 9.055385138137417 5.830951894845301 3.1622776601683795 10.295630140987 1 6.082762530298219
6.324555320336759 2.23606797749979 3 6 7.280109889280518 5 8 7.0710678118654755 0 8.602325267042627 1 2 4.47213595499958 5.830951894845301 10 2.8284271247461903 5.830951894845301 11.40175425099138 5.385164807134504 8.246211251235321 2 3.605551275463989 10.198039027185569 5.656854249492381 9.219544457292887 12.806248474865697 7.211102550927978 4 6.324555320336759 7 1
5.0990195135927845 9.219544457292887 7.280109889280518 7.0710678118654755 7 7 7.615773105863909 6.324555320336759 8.602325267042627 0 9.219544457292887 9.899494936611665 7.615773105863909 2.8284271247461903 1.4142135623730951 8.602325267042627 4 2.8284271247461903 7.280109889280518 7.0710678118654755 7.615773105863909 8.06225774829855 7.615773105863909 3.1622776601683795 7.280109889280518 4.242640687119285 1.4142135623730951 7.0710678118654755 7.0710678118654755 7.280109889280518 8.06225774829855
7.280109889280518 1.4142135623730951 4 7 7.0710678118654755 6 9 8.06225774829855 1 9.219544457292887 0 1 4.123105625617661 6.4031242374328485 10.63014581273465 2.23606797749979 6.708203932499369 12.041594578792296 5.0990195135927845 8.06225774829855 3 3.1622776601683795 10.04987562112089 6.4031242374328485 9.055385138137417 13.45362404707371 7.810249675906654 5 6.082762530298219 8 2
8.246211251235321 1 5 8 7 7 10 9.055385138137417 2 9.899494936611665 1 0 4 7.0710678118654755 11.313708498984761 2 7.615773105863909 12.727922061357855 5 8 4 3 10 7.211102550927978 9 14.142135623730951 8.48528137423857 6 6 9 3
8.246211251235321 3 6.4031242374328485 8.94427190999916 3 8.06225774829855 10.770329614269007 9.486832980505138 4.47213595499958 7.615773105863909 4.123105625617661 4 0 5.0990195135927845 8.94427190999916 2 7.0710678118654755 10.295630140987 1 4 5.656854249492381 1 6 6 5 11.661903789690601 6.324555320336759 7.211102550927978 2 9.848857801796104 5
4.242640687119285 6.4031242374328485 5 5.830951894845301 5.385164807134504 5.385164807134504 7.0710678118654755 5.656854249492381 5.830951894845301 2.8284271247461903 6.4031242374328485 7.0710678118654755 5.0990195135927845 0 4.242640687119285 5.830951894845301 2.8284271247461903 5.656854249492381 5 5.830951894845301 5.0990195135927845 5.385164807134504 7.0710678118654755 1.4142135623730951 6.4031242374328485 7.0710678118654755 1.4142135623730951 5.0990195135927845 5.0990195135927845 6.4031242374328485 5.385164807134504
6 10.63014581273465 8.54400374531753 8 8.06225774829855 8.06225774829855 8.246211251235321 7.0710678118654755 10 1.4142135623730951 10.63014581273465 11.313708498984761 8.94427190999916 4.242640687119285 0 10 5.0990195135927845 1.4142135623730951 8.54400374531753 8 8.94427190999916 9.433981132056603 8.246211251235321 4.47213595499958 8.06225774829855 2.8284271247461903 2.8284271247461903 8.246211251235321 8.246211251235321 8.06225774829855 9.433981132056603
8 1 5.385164807134504 8.246211251235321 5 7.280109889280518 10.198039027185569 9.055385138137417 2.8284271247461903 8.602325267042627 2.23606797749979 2 2 5.830951894845301 10 0 7.0710678118654755 11.40175425099138 3 6 4.47213595499958 1 8 6.324555320336759 7 12.806248474865697 7.211102550927978 6.324555320336759 4 9.219544457292887 3.605551275463989
1.4142135623730951 7.280109889280518 3.605551275463989 3.1622776601683795 8.06225774829855 3 4.242640687119285 2.8284271247461903 5.830951894845301 4 6.708203932499369 7.615773105863909 7.0710678118654755 2.8284271247461903 5.0990195135927845 7.0710678118654755 0 6.324555320336759 7.280109889280518 8.602325267042627 4.242640687119285 7 9.899494936611665 1.4142135623730951 9.219544457292887 7.615773105863909 3.1622776601683795 3.1622776601683795 7.615773105863909 3.605551275463989 5
7.0710678118654755 12.041594578792296 9.848857801796104 9.055385138137417 9.219544457292887 9.219544457292887 9.055385138137417 8 11.40175425099138 2.8284271247461903 12.041594578792296 12.727922061357855 10.295630140987 5.656854249492381 1.4142135623730951 11.40175425099138 6.324555320336759 0 9.848857801796104 9.055385138137417 10.295630140987 10.816653826391969 9.055385138137417 5.830951894845301 9 1.4142135623730951 4.242640687119285 9.486832980505138 9.486832980505138 9 10.816653826391969
8.54400374531753 4 7.0710678118654755 9.433981132056603 2 8.602325267042627 11.180339887498949 9.848857801796104 5.385164807134504 7.280109889280518 5.0990195135927845 5 1 5 8.54400374531753 3 7.280109889280518 9.848857801796104 0 3 6.4031242374328485 2 5 6.082762530298219 4 11.180339887498949 6.082762530298219 7.810249675906654 1 10.295630140987 5.830951894845301
10 7 9.433981132056603 11.313708498984761 1 10.63014581273465 12.806248474865697 11.40175425099138 8.246211251235321 7.0710678118654755 8.06225774829855 8 4 5.830951894845301 8 6 8.602325267042627 9.055385138137417 3 0 8.94427190999916 5 2 7.211102550927978 1 10.198039027185569 6.324555320336759 10 2 12.041594578792296 8.54400374531753
4.47213595499958 4.123105625617661 1 4 8.06225774829855 3 6 5.0990195135927845 2 7.615773105863909 3 4 5.656854249492381 5.0990195135927845 8.94427190999916 4.47213595499958 4.242640687119285 10.295630140987 6.4031242374328485 8.94427190999916 0 5 10.770329614269007 4.47213595499958 9.848857801796104 11.661903789690601 6.324555320336759 2 7.211102550927978 5 1
8.06225774829855 2 5.830951894845301 8.54400374531753 4 7.615773105863909 10.44030650891055 9.219544457292887 3.605551275463989 8.06225774829855 3.1622776601683795 3 1 5.385164807134504 9.433981132056603 1 7 10.816653826391969 2 5 5 0 7 6.082762530298219 6 12.206555615733702 6.708203932499369 6.708203932499369 3 9.486832980505138 4.242640687119285
11.313708498984761 9 11.180339887498949 12.806248474865697 3 12.206555615733702 14.142135623730951 12.727922061357855 10.198039027185569 7.615773105863909 10.04987562112089 10 6 7.0710678118654755 8.246211251235321 8 9.899494936611665 9.055385138137417 5 2 10.770329614269007 7 0 8.48528137423857 1 10 7.211102550927978 11.661903789690601 4 13.45362404707371 10.44030650891055
2.8284271247461903 6.708203932499369 4.123105625617661 4.47213595499958 6.708203932499369 4.123105625617661 5.656854249492381 4.242640687119285 5.656854249492381 3.1622776601683795 6.4031242374328485 7.211102550927978 6 1.4142135623730951 4.47213595499958 6.324555320336759 1.4142135623730951 5.830951894845301 6.082762530298219 7.211102550927978 4.47213595499958 6.082762530298219 8.48528137423857 0 7.810249675906654 7.211102550927978 2 4 6.324555320336759 5 5
10.63014581273465 8 10.295630140987 12.041594578792296 2 11.40175425099138 13.45362404707371 12.041594578792296 9.219544457292887 7.280109889280518 9.055385138137417 9 5 6.4031242374328485 8.06225774829855 7 9.219544457292887 9 4 1 9.848857801796104 6 1 7.810249675906654 0 10.04987562112089 6.708203932499369 10.816653826391969 3 12.727922061357855 9.486832980505138
8.246211251235321 13.45362404707371 11.180339887498949 10.198039027185569 10.44030650891055 10.44030650891055 10 9.055385138137417 12.806248474865697 4.242640687119285 13.45362404707371 14.142135623730951 11.661903789690601 7.0710678118654755 2.8284271247461903 12.806248474865697 7.615773105863909 1.4142135623730951 11.180339887498949 10.198039027185569 11.661903789690601 12.206555615733702 10 7.211102550927978 10.04987562112089 0 5.656854249492381 10.770329614269007 10.770329614269007 10.04987562112089 12.206555615733702
4.47213595499958 7.810249675906654 6.082762530298219 6.324555320336759 6.082762530298219 6.082762530298219 7.211102550927978 5.830951894845301 7.211102550927978 1.4142135623730951 7.810249675906654 8.48528137423857 6.324555320336759 1.4142135623730951 2.8284271247461903 7.211102550927978 3.1622776601683795 4.242640687119285 6.082762530298219 6.324555320336759 6.324555320336759 6.708203932499369 7.211102550927978 2 6.708203932499369 5.656854249492381 0 6 6 6.708203932499369 6.708203932499369
2.8284271247461903 6.082762530298219 1 2 9.219544457292887 1 4 3.1622776601683795 4 7.0710678118654755 5 6 7.211102550927978 5.0990195135927845 8.246211251235321 6.324555320336759 3.1622776601683795 9.486832980505138 7.810249675906654 10 2 6.708203932499369 11.661903789690601 4 10.816653826391969 10.770329614269007 6 0 8.48528137423857 3 3
8.94427190999916 5 7.810249675906654 10 1 9.219544457292887 11.661903789690601 10.295630140987 6.324555320336759 7.0710678118654755 6.082762530298219 6 2 5.0990195135927845 8.246211251235321 4 7.615773105863909 9.486832980505138 1 2 7.211102550927978 3 4 6.324555320336759 3 10.770329614269007 6 8.48528137423857 0 10.816653826391969 6.708203932499369
2.23606797749979 9.055385138137417 4 1 11.40175425099138 2 1 1 7 7.280109889280518 8 9 9.848857801796104 6.4031242374328485 8.06225774829855 9.219544457292887 3.605551275463989 9 10.295630140987 12.041594578792296 5 9.486832980505138 13.45362404707371 5 12.727922061357855 10.04987562112089 6.708203932499369 3 10.816653826391969 0 6
5.385164807134504 3.1622776601683795 2 5 7.615773105863909 4 7 6.082762530298219 1 8.06225774829855 2 3 5 5.385164807134504 9.433981132056603 3.605551275463989 5 10.816653826391969 5.830951894845301 8.54400374531753 1 4.242640687119285 10.44030650891055 5 9.486832980505138 12.206555615733702 6.708203932499369 3 6.708203932499369 6 0

31
8 8
0 9
5 10
8 10
0 3
7 10
10 10
9 9
2 10
7 3
1 10
0 10
0 6
5 5
8 2
0 8
7 7
9 1
0 5
0 2
4 10
0 7
0 0
6 6
0 1
10 0
6 4
6 10
0 4
9 10
3 10

 */
