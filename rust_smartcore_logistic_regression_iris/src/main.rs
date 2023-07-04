#[allow(non_snake_case)]
use smartcore::dataset::iris::load_dataset;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;

fn main() {
    let iris_data = load_dataset();

    // Extract the features and target variable.

    // The features variable.
    // Transform the dataset into a NxM matrix.
    let x = DenseMatrix::new(
        iris_data.num_samples,  //nrows
        iris_data.num_features, // ncols
        iris_data.data,         // values
        false,                  // column_major
    );

    // The target variable.
    let y = iris_data.target;

    // Split dataset into training/test (80%/20%).
    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, 0.2, true, None);

    // Create a logistic regression model and fit it to the data.
    let lr_model = LogisticRegression::fit(&x_train, &y_train, Default::default()).unwrap();

    // Use the model to predict the target variable for new data.
    let y_pred = lr_model.predict(&x_test).unwrap();

    // Calculate the accuracy of the model.
    let acc = accuracy(&y_test, &y_pred);

    // Print the accuracy of the model.
    println!("Accuracy: {:.2}%", acc * 100.0);
}
