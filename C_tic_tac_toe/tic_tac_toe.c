#include <stdio.h>
#include <stdlib.h>

// Initialisation du plateau
char* initialize_board(int size) {
    char* board = (char*)malloc(size * size * sizeof(char));
    if (!board) {
        fprintf(stderr, "Erreur : Allocation mémoire échouée\n");
        exit(EXIT_FAILURE);
    }
    for (int i = 0; i < size * size; ++i) {
        board[i] = ' ';
    }
    return board;
}

// Affichage du plateau
void display_board(char* board, int size) {
    for (int i = 0; i < size; ++i) {
        for (int j = 0; j < size; ++j) {
            printf(" %c ", board[i * size + j]);
            if (j < size - 1) printf("|");
        }
        printf("\n");
        if (i < size - 1) {
            for (int j = 0; j < size; ++j) {
                printf("---");
                if (j < size - 1) printf("+");
            }
            printf("\n");
        }
    }
}

// Vérification de victoire
int check_winner(char* board, int size, char player) {
    // Vérifier les lignes et colonnes
    for (int i = 0; i < size; ++i) {
        int row_win = 1, col_win = 1;
        for (int j = 0; j < size; ++j) {
            if (board[i * size + j] != player) row_win = 0;
            if (board[j * size + i] != player) col_win = 0;
        }
        if (row_win || col_win) return 1;
    }

    // Vérifier les diagonales
    int diag1_win = 1, diag2_win = 1;
    for (int i = 0; i < size; ++i) {
        if (board[i * size + i] != player) diag1_win = 0;
        if (board[i * size + (size - i - 1)] != player) diag2_win = 0;
    }
    return diag1_win || diag2_win;
}

// Libération de la mémoire
void free_board(char* board) {
    free(board);
    printf("Plateau libere.\n");
}

// Jeu principal
void play_game(int size) {
    char* board = initialize_board(size);
    char current_player = 'X';
    int moves = 0, max_moves = size * size;

    while (1) {
        system("clear"); // Pour un affichage clair (ou utilisez `cls` sur Windows)
        display_board(board, size);
        printf("Joueur %c, entrez votre coup (ligne et colonne) : ", current_player);

        int row, col;
        scanf("%d %d", &row, &col);
        if (row < 0 || row >= size || col < 0 || col >= size || board[row * size + col] != ' ') {
            printf("Coup invalide. Réessayez.\n");
            getchar(); // Consommer le retour à la ligne
            continue;
        }

        board[row * size + col] = current_player;
        moves++;

        if (check_winner(board, size, current_player)) {
            system("clear");
            display_board(board, size);
            printf("Felicitations ! Joueur %c a gagne !\n", current_player);
            break;
        }

        if (moves == max_moves) {
            system("clear");
            display_board(board, size);
            printf("Match nul !\n");
            break;
        }

        // Changer de joueur
        current_player = (current_player == 'X') ? 'O' : 'X';
    }

    free_board(board);
}

int main() {
    int size;
    printf("Entrez la taille du plateau (ex. 3 pour 3x3) : ");
    scanf("%d", &size);
    play_game(size);
    return 0;
}