use super::*;

macro_rules! generate_value_constraint_tests {
    (
        tests: [
            $({
                name: $ti:ident,
                settings: $st:ty => $se:expr $(,)?
            }),+ $(,)?
        ],
        constraints: $ct:ty => $ce:expr,
        expected: $e:expr $(,)?
    ) => {
        generate_value_constraint_tests!(
            tests: [
                $({
                    name: $ti,
                    settings: $st => $se,
                    constraints: $ct => $ce,
                }),+
            ],
            expected: $e
        );
    };
    (
        tests: [
            $({
                name: $ti:ident,
                settings: $st:ty => $se:expr,
                constraints: $ct:ty => $ce:expr $(,)?
            }),+ $(,)?
        ],
        expected: $e:expr $(,)?
    ) => {
        generate_value_constraint_tests!(
            tests: [
                $({
                    name: $ti,
                    settings: $st => $se,
                    constraints: $ct => $ce,
                }),+
            ],
            validate: |result| {
                assert_eq!(result, $e);
            }
        );
    };
    (
        tests: [
            $({
                name: $ti:ident,
                settings: $st:ty => $se:expr,
                constraints: $ct:ty => $ce:expr $(,)?
            }),+ $(,)?
        ],
        validate: |$a:ident| $b:block
    ) => {
        $(
            #[test]
            fn $ti() {
                test_value_constraint!(
                    settings: $st => $se,
                    constraints: $ct => $ce,
                    validate: |$a| $b
                );
            }
        )+
    };
}

macro_rules! test_value_constraint {
    (
        settings: $st:ty => $se:expr,
        constraints: $ct:ty => $ce:expr,
        expected: $e:expr $(,)?
    ) => {
        test_value_constraint!(
            settings: $st => $se,
            constraints: $ct => $ce,
            validate: |result| {
                assert_eq!(result, $e);
            }
        );
    };
    (
        settings: $st:ty => $se:expr,
        constraints: $ct:ty => $ce:expr,
        validate: |$a:ident| $b:block
    ) => {{
        let settings: &[Option<$st>] = $se;
        let constraints: &[ResolvedValueConstraint<$ct>] = $ce;

        for constraint in constraints {
            for setting in settings {
                let closure = |$a| $b;
                let actual = constraint.fitness_distance(setting.as_ref());
                closure(actual);
            }
        }
    }};
    (
        checks: [
            $({
                setting: $st:ty => $se:expr,
                constraint: $ct:ty => $ce:expr,
                expected: $ee:expr $(,)?
            }),+ $(,)?
        ]
    ) => {
        test_value_constraint!(
            checks: [
                $({
                    setting: $st => $se,
                    constraint: $ct => $ce,
                    expected: $ee,
                }),+
            ],
            validate: |actual, expected| {
                assert_eq!(actual, expected);
            }
        );
    };
    (
        checks: [
            $({
                setting: $st:ty => $se:expr,
                constraint: $ct:ty => $ce:expr,
                expected: $ee:expr $(,)?
            }),+ $(,)?
        ],
        validate: |$ai:ident, $ei:ident| $b:block
    ) => {{
        $({
            let closure = |$ai, $ei| $b;
            let actual = $ce.fitness_distance($se.as_ref());
            closure(actual, $ee);
        })+
    }};
}

pub mod bool;
pub mod f64;
pub mod string;
pub mod u64;
