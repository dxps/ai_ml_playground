from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.linear_model import LogisticRegression

# List of possible questions and answers
questions = [
    "What is your return policy?",
    "How can I track my order?",
    "Do you offer international shipping?",
    # Add more questions here
]

answers = [
    "Our return policy allows returns within 30 days of purchase.",
    "You can track your order by logging into your account and visiting the order tracking page.",
    "Yes, we offer international shipping to select countries.",
    # Add corresponding answers here
]

# Vectorize the questions using TF-IDF
vectorizer = TfidfVectorizer()
question_vectors = vectorizer.fit_transform(questions)

# Prepare the training data for intent classification
X_train = question_vectors
y_train = range(len(questions))

# Train an intent classification model.
classifier = LogisticRegression()
classifier.fit(X_train, y_train)

# Function to find the best answer based on intent classification
def get_best_answer(input_question):
    input_vector = vectorizer.transform([input_question])
    predicted_intent = classifier.predict(input_vector)[0]
    return answers[predicted_intent]

# Example usage
customer_question = input("Please enter your question: ")
best_answer = get_best_answer(customer_question)
print("Answer:", best_answer)
